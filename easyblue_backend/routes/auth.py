from flask import Blueprint, request, jsonify
from werkzeug.security import generate_password_hash, check_password_hash
from functools import wraps
from app.models import db, User, RiderProfile, VendorProfile
import jwt
import datetime
import os

auth_bp = Blueprint('auth', __name__)

# Secret key for JWT signing (Should be in your .env)
SECRET_KEY = os.getenv("SECRET_KEY", "easyblue_dev_secret_12345")

# --- DECORATORS FOR ENTRY RESTRICTIONS ---

def token_required(f):
    @wraps(f)
    def decorated(*args, **kwargs):
        token = request.headers.get('Authorization')
        if not token:
            return jsonify({'message': 'Token is missing!'}), 401
        try:
            # Assumes 'Bearer <token>' format
            token = token.split(" ")[1]
            data = jwt.decode(token, SECRET_KEY, algorithms=["HS256"])
            current_user = User.query.filter_by(id=data['user_id']).first()
        except:
            return jsonify({'message': 'Token is invalid!'}), 401
        
        return f(current_user, *args, **kwargs)
    return decorated

def role_required(required_role):
    def decorator(f):
        @wraps(f)
        def decorated_function(current_user, *args, **kwargs):
            if current_user.role != required_role:
                return jsonify({'message': 'Access denied: Unauthorized role'}), 403
            return f(current_user, *args, **kwargs)
        return decorated_function
    return decorator

# --- AUTHENTICATION ROUTES ---

@auth_bp.route('/register', methods=['POST'])
def register():
    data = request.get_json()
    
    if User.query.filter_by(email=data.get('email')).first():
        return jsonify({"message": "Email already registered"}), 400

    # 1. Create the base User (Locked by default)
    new_user = User(
        email=data.get('email'),
        password_hash=generate_password_hash(data.get('password'), method='pbkdf2:sha256'),
        role=data.get('role'), 
        full_name=data.get('full_name'),
        phone=data.get('phone'),
        is_approved=False  # Gate: Admin must approve
    )
    
    db.session.add(new_user)
    db.session.flush()

    # 2. Role-Specific Profile Creation
    if data.get('role') == 'rider':
        new_profile = RiderProfile(
            user_id=new_user.id,
            vehicle_type=data.get('vehicle_type', 'Motorcycle')
        )
        db.session.add(new_profile)
        
    elif data.get('role') == 'vendor':
        new_profile = VendorProfile(
            user_id=new_user.id,
            business_name=data.get('business_name'),
            business_address=data.get('business_address')
        )
        db.session.add(new_profile)

    db.session.commit()
    return jsonify({"message": "Registration successful. Waiting for Overseer approval."}), 201

@auth_bp.route('/login', methods=['POST'])
def login():
    data = request.get_json()
    user = User.query.filter_by(email=data.get('email')).first()

    if not user or not check_password_hash(user.password_hash, data.get('password')):
        return jsonify({"message": "Invalid credentials"}), 401

    # Check the Overseer Gate
    if not user.is_approved:
        return jsonify({"message": "Access restricted: Pending admin approval"}), 403

    # Generate JWT for Dioxus Frontend
    token = jwt.encode({
        'user_id': user.id,
        'role': user.role,
        'exp': datetime.datetime.utcnow() + datetime.timedelta(hours=24)
    }, SECRET_KEY, algorithm="HS256")

    return jsonify({
        "token": token,
        "user": {
            "id": user.id,
            "name": user.full_name,
            "role": user.role
        }
    }), 200

# --- ADMIN OVERSEER ROUTE EXAMPLE ---

@auth_bp.route('/approve-user/<user_id>', methods=['POST'])
@token_required
@role_required('admin')
def approve_user(current_user, user_id):
    """The Admin 'Overseer' flips the switch here"""
    target_user = User.query.get(user_id)
    if not target_user:
        return jsonify({"message": "User not found"}), 404
    
    target_user.is_approved = True
    db.session.commit()
    return jsonify({"message": f"User {target_user.full_name} approved."}), 200
