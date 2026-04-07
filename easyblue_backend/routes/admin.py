from flask import Blueprint, jsonify, request
from app.models import db, User, RiderProfile, VendorProfile, Order
from app.routes.auth import token_required, role_required

admin_bp = Blueprint('admin', __name__)

# --- CATEGORIZED HISTORY LOGIC ---

@admin_bp.route('/history/riders', methods=['GET'])
@token_required
@role_required('admin')
def get_all_riders_history(current_user):
    """View all riders and their delivery performance counts"""
    riders = RiderProfile.query.all()
    history = []
    for r in riders:
        # Count orders accepted/completed by this specific rider
        completed_count = Order.query.filter_by(rider_id=r.id, status='delivered').count()
        active_count = Order.query.filter_by(rider_id=r.id, status='accepted').count()
        
        history.append({
            "rider_name": r.user.full_name,
            "rider_id": r.id,
            "total_completed": completed_count,
            "currently_handling": active_count,
            "phone": r.user.phone
        })
    return jsonify(history), 200

@admin_bp.route('/history/vendors', methods=['GET'])
@token_required
@role_required('admin')
def get_all_vendors_history(current_user):
    """View all registered vendors and their total booking volume"""
    vendors = VendorProfile.query.all()
    history = []
    for v in vendors:
        order_count = Order.query.filter_by(vendor_id=v.id).count()
        history.append({
            "business_name": v.business_name,
            "vendor_id": v.id,
            "owner": v.user.full_name,
            "total_orders_placed": order_count,
            "category": v.category
        })
    return jsonify(history), 200

@admin_bp.route('/history/customers', methods=['GET'])
@token_required
@role_required('admin')
def get_standard_customer_history(current_user):
    """View history for standard (non-vendor) customers"""
    # We filter for orders where vendor_id is null (placed by individual users)
    standard_orders = Order.query.filter(Order.vendor_id == None).all()
    return jsonify([{
        "order_id": o.id,
        "customer_name": o.user.full_name if o.customer_id else "Guest",
        "pickup": o.pickup_address,
        "dropoff": o.dropoff_address,
        "date": o.created_at.isoformat()
    } for o in standard_orders]), 200

# --- INDIVIDUAL DRILL-DOWN ---

@admin_bp.route('/history/rider/<int:rider_id>', methods=['GET'])
@token_required
@role_required('admin')
def get_specific_rider_log(current_user, rider_id):
    """The 'Scanned History' for a specific individual rider"""
    orders = Order.query.filter_by(rider_id=rider_id).order_by(Order.created_at.desc()).all()
    return jsonify([{
        "id": o.id,
        "status": o.status,
        "pickup": o.pickup_address,
        "dropoff": o.dropoff_address,
        "timestamp": o.completed_at.isoformat() if o.completed_at else o.created_at.isoformat()
    } for o in orders]), 200
