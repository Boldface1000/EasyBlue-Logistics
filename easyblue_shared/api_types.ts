// easyblue/easyblue_shared/types/api_types.ts

// The shared structure of an order for Standard Customers and Vendors
export interface OrderBooking {
    id?: string;               // Optional (Server generates on creation)
    vendor_id?: string;        // Only present if placed by a Vendor
    customer_name: string;
    customer_phone: string;
    pickup_address: string;
    dropoff_address: string;
    is_simultaneous: boolean;   // Handles logic for complex routing
    status: 'pending' | 'accepted' | 'in_transit' | 'delivered' | 'cancelled';
}

// The shared structure of a User Profile (Admin, Rider, Vendor)
export interface UserProfile {
    id: string;
    email: string;
    role: 'admin' | 'rider' | 'vendor' | 'customer';
    name: string;
    phone: string;
    is_approved: boolean;       // Critical flag for overseer logic
    last_known_location?: {     // Real-time tracking data
        lat: number;
        lng: number;
        timestamp: number;
    };
}

