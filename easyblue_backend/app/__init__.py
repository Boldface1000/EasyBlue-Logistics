from flask import Flask
from flask_sqlalchemy import SQLAlchemy
from flask_migrate import Migrate
from flask_cors import CORS
import sentry_sdk
from sentry_sdk.integrations.flask import FlaskIntegration
import os
from dotenv import load_dotenv

# Load environment variables (DNS, DB URL, Sentry DSN)
load_dotenv()

# Initialize extensions
db = SQLAlchemy()
migrate = Migrate()
cors = CORS()

def create_app():
    # Sentry Error Tracking Initialization
    sentry_sdk.init(
        dsn=os.getenv("SENTRY_DSN"),
        integrations=[FlaskIntegration()],
        traces_sample_rate=1.0
    )

    app = Flask(__name__)

    # Configuration - Using PostgreSQL
    app.config['SQLALCHEMY_DATABASE_URI'] = os.getenv("DATABASE_URL")
    app.config['SQLALCHEMY_TRACK_MODIFICATIONS'] = False
    app.config['SECRET_KEY'] = os.getenv("SECRET_KEY")

    # Bind extensions to app
    db.init_app(app)
    migrate.init_app(app, db)
    cors.init_app(app)

    # Import models here to ensure they are registered with SQLAlchemy
    from app import models

    # Register Blueprints (Routes)
    from app.routes.auth import auth_bp
    from app.routes.orders import orders_bp
    
    app.register_blueprint(auth_bp, url_prefix='/api/auth')
    app.register_blueprint(orders_bp, url_prefix='/api/orders')

    return app

