from flask import Blueprint, request, jsonify
from app.models import db, Order, VendorProfile, User
from app.routes.auth import token_required

booking_bp = Blueprint('booking', __name__)

@booking_bp.route('/create', methods=['POST'])
@token_required
def create_booking(current_user):
    """
    Handles bookings for:
    1. Standard Customers (current_user.role == 'customer')
    2. Vendors (current_user.role == 'vendor')
    """
    data = request.get_json()
    
    # Identify if this is a Vendor placing the order
    vendor = None
    if current_user.role == 'vendor':
        vendor = VendorProfile.query.filter_by(user_id=current_user.id).first()

    # Determine Order Type: 'standard' or 'simultaneous'
    order_type = data.get('order_type', 'standard')

    new_order = Order(
        customer_id=current_user.id if current_user.role == 'customer' else None,
        vendor_id=vendor.id if vendor else None,
        order_type=order_type,
        status='pending',
        
        # Location Data
        pickup_address=data.get('pickup_address'),
        pickup_lat=data.get('pickup_lat'),
        pickup_lng=data.get('pickup_lng'),
        
        dropoff_address=data.get('dropoff_address'),
        dropoff_lat=data.get('dropoff_lat'),
        dropoff_lng=data.get('dropoff_lng'),
        
        # Meta
        price=data.get('price', 0.0)
    )

    db.session.add(new_order)
    db.session.commit()

    return jsonify({
        "message": "Booking placed successfully",
        "order_id": new_order.id,
        "type": new_order.order_type
    }), 201

@booking_bp.route('/available', methods=['GET'])
@token_required
def get_available_orders(current_user):
    """Riders use this to see 'pending' orders they can accept"""
    if current_user.role != 'rider':
        return jsonify({"message": "Unauthorized"}), 403
        
    # Only show orders that haven't been picked up yet
    orders = Order.query.filter_by(status='pending').all()
    
    return jsonify([{
        "id": o.id,
        "type": o.order_type,
        "pickup": o.pickup_address,
        "dropoff": o.dropoff_address,
        "is_vendor_order": True if o.vendor_id else False
    } for o in orders]), 200

@booking_bp.route('/accept/<order_id>', methods=['POST'])
@token_required
def accept_order(current_user, order_id):
    """Rider accepts an order and it becomes 'In Progress'"""
    if current_user.role != 'rider':
        return jsonify({"message": "Only riders can accept orders"}), 403

    order = Order.query.get(order_id)
    if not order or order.status != 'pending':
        return jsonify({"message": "Order unavailable"}), 404

    # Link the order to the Rider's Profile
    rider_profile = current_user.rider_profile
    order.rider_id = rider_profile.id
    order.status = 'accepted'
    
    db.session.commit()

    # NOTE: This is where we would trigger a push notification 
    # to the Vendor/Customer that their rider is on the way.
    
    return jsonify({
        "message": "Order accepted",
        "order_status": order.status
    }), 200
