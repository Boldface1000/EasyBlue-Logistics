from app.routes.auth import auth_bp
# We will add the following once the scripts are created:
# from app.routes.orders import orders_bp
# from app.routes.tracking import tracking_bp

# This list makes it easy to iterate and register in the main __init__.py if needed
all_blueprints = [
    (auth_bp, '/api/auth'),
    # (orders_bp, '/api/orders'),
    # (tracking_bp, '/api/tracking'),
]
