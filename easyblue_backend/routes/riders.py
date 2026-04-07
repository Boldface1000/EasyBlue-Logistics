from flask import Blueprint, request, jsonify
from app.models import db, RiderProfile, Order, TrackingLog
from app.routes.auth import token_required, role_required
from datetime import datetime

riders_bp = Blueprint('riders', __name__)

@riders_bp.route('/update-location', methods=['POST'])
@token_required
@role_required('rider')
def update_location(current_user):
    """
    High-frequency endpoint for real-time tracking.
    Dioxus PWA will call this every 30-60 seconds while an order is active.
    """
    data = request.get_json()
    lat = data.get('lat')
    lng = data.get('lng')
    active_order_id = data.get('order_id')

    rider_profile = current_user.rider_profile
    if not rider_profile:
        return jsonify({"message": "Rider profile not found"}), 404

    # 1. Update the Rider's current "Live" position
    rider_profile.current_lat = lat
    rider_profile.current_lng = lng
    rider_profile.last_updated = datetime.utcnow()
    rider_profile.is_active = True

    # 2. If they are on a delivery, log a breadcrumb for the Admin's Map
    if active_order_id:
        log = TrackingLog(
            order_id=active_order_id,
            rider_id=rider_profile.id,
            lat=lat,
            lng=lng
        )
        db.session.add(log)

    db.session.commit()
    return jsonify({"status": "Location synced"}), 200

@riders_bp.route('/my-active-orders', methods=['GET'])
@token_required
@role_required('rider')
def get_my_orders(current_user):
    """Shows the rider what they are currently delivering"""
    rider_id = current_user.rider_profile.id
    orders = Order.query.filter_by(rider_id=rider_id).filter(Order.status != 'delivered').all()
    
    return jsonify([{
        "id": o.id,
        "type": o.order_type,
        "status": o.status,
        "pickup": o.pickup_address,
        "dropoff": o.dropoff_address,
        "customer_phone": o.user.phone if o.customer_id else "Vendor Client"
    } for o in orders]), 200

@riders_bp.route('/complete-order/<order_id>', methods=['POST'])
@token_required
@role_required('rider')
def complete_order(current_user, order_id):
    """Finalizes the delivery and stops the tracking log for this order"""
    order = Order.query.get(order_id)
    
    if not order or order.rider_id != current_user.rider_profile.id:
        return jsonify({"message": "Unauthorized or order not found"}), 403

    order.status = 'delivered'
    order.completed_at = datetime.utcnow()
    
    # We can also set the rider to 'inactive' if they want to stop receiving pings
    # current_user.rider_profile.is_active = False 
    
    db.session.commit()
    return jsonify({"message": "Order marked as delivered. Success!"}), 200
