from flask_sqlalchemy import SQLAlchemy
from datetime import datetime
from uuid import uuid4

db = SQLAlchemy()

def get_uuid():
    return uuid4().hex

class User(db.Model):
    __tablename__ = 'users'
    id = db.Column(db.String(32), primary_key=True, default=get_uuid)
    email = db.Column(db.String(120), unique=True, nullable=False)
    password_hash = db.Column(db.String(256), nullable=False)
    role = db.Column(db.String(20), nullable=False) # 'admin', 'rider', 'vendor', 'customer'
    full_name = db.Column(db.String(100), nullable=False)
    phone = db.Column(db.String(20), nullable=False)
    is_approved = db.Column(db.Boolean, default=False) # Overseer approval flag
    created_at = db.Column(db.DateTime, default=datetime.utcnow)

    # Relationships
    rider_profile = db.relationship('RiderProfile', backref='user', uselist=False)
    vendor_profile = db.relationship('VendorProfile', backref='user', uselist=False)

class RiderProfile(db.Model):
    __tablename__ = 'rider_profiles'
    id = db.Column(db.Integer, primary_key=True)
    user_id = db.Column(db.String(32), db.ForeignKey('users.id'), nullable=False)
    vehicle_type = db.Column(db.String(50)) # e.g., 'Bike', 'Van'
    is_active = db.Column(db.Boolean, default=False) # Toggle for "Accepting Orders"
    current_lat = db.Column(db.Float, nullable=True)
    current_lng = db.Column(db.Float, nullable=True)
    last_updated = db.Column(db.DateTime, onupdate=datetime.utcnow)

class VendorProfile(db.Model):
    __tablename__ = 'vendor_profiles'
    id = db.Column(db.Integer, primary_key=True)
    user_id = db.Column(db.String(32), db.ForeignKey('users.id'), nullable=False)
    business_name = db.Column(db.String(150), nullable=False)
    business_address = db.Column(db.Text, nullable=False)
    category = db.Column(db.String(50)) # e.g., 'Retail', 'Food'

class Order(db.Model):
    __tablename__ = 'orders'
    id = db.Column(db.String(32), primary_key=True, default=get_uuid)
    
    # Ownership
    customer_id = db.Column(db.String(32), db.ForeignKey('users.id'), nullable=True) # For registered users
    vendor_id = db.Column(db.Integer, db.ForeignKey('vendor_profiles.id'), nullable=True)
    rider_id = db.Column(db.Integer, db.ForeignKey('rider_profiles.id'), nullable=True)

    # Logic
    order_type = db.Column(db.String(20), default='standard') # 'standard', 'simultaneous'
    status = db.Column(db.String(20), default='pending') # 'pending', 'accepted', 'in_transit', 'delivered'
    
    # Location Details
    pickup_address = db.Column(db.Text, nullable=False)
    pickup_lat = db.Column(db.Float)
    pickup_lng = db.Column(db.Float)
    
    dropoff_address = db.Column(db.Text, nullable=False)
    dropoff_lat = db.Column(db.Float)
    dropoff_lng = db.Column(db.Float)

    # Logistics Meta
    price = db.Column(db.Float, default=0.0)
    created_at = db.Column(db.DateTime, default=datetime.utcnow)
    completed_at = db.Column(db.DateTime, nullable=True)

class TrackingLog(db.Model):
    """History for the Admin to read movements in realtime"""
    __tablename__ = 'tracking_logs'
    id = db.Column(db.Integer, primary_key=True)
    order_id = db.Column(db.String(32), db.ForeignKey('orders.id'), nullable=False)
    rider_id = db.Column(db.Integer, db.ForeignKey('rider_profiles.id'), nullable=False)
    lat = db.Column(db.Float, nullable=False)
    lng = db.Column(db.Float, nullable=False)
    timestamp = db.Column(db.DateTime, default=datetime.utcnow)
