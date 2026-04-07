import os
import requests

def send_rider_alert(rider_phone, message):
    """
    Triggers an SMS or Push Notification.
    Since you mentioned Termux, you could even hook this 
    into a Termux:API script later.
    """
    # Placeholder for an SMS Gateway or Firebase Cloud Messaging
    print(f"ALERT SENT TO {rider_phone}: {message}")
    return True
