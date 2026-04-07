import math

def calculate_distance(lat1, lon1, lat2, lon2):
    """
    Haversine formula to calculate the distance between 
    pickup and dropoff in kilometers.
    """
    R = 6371  # Earth radius in km
    dlat = math.radians(lat2 - lat1)
    dlon = math.radians(lon2 - lon1)
    a = math.sin(dlat/2)**2 + math.cos(math.radians(lat1)) * \
        math.cos(math.radians(lat2)) * math.sin(dlon/2)**2
    c = 2 * math.atan2(math.sqrt(a), math.sqrt(1-a))
    return R * c

def calculate_price(distance_km, order_type='standard'):
    """
    Dynamic pricing logic for EasyBlue.
    Simultaneous orders might have a base discount or extra fee.
    """
    base_fare = 500.0  # Example: 500 Naira base
    rate_per_km = 150.0
    
    total = base_fare + (distance_km * rate_per_km)
    
    if order_type == 'simultaneous':
        total *= 1.5  # 50% extra for complex multi-stop routing
        
    return round(total, 2)
