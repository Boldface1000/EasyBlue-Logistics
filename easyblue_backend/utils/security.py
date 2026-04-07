import jwt
import os
from datetime import datetime, timedelta

def generate_token(user_id, role):
    SECRET_KEY = os.getenv("SECRET_KEY")
    payload = {
        'user_id': user_id,
        'role': role,
        'exp': datetime.utcnow() + timedelta(hours=24)
    }
    return jwt.encode(payload, SECRET_KEY, algorithm="HS256")
