# CORS Fix Applied ✅

## Problem
The frontend (localhost:3000) couldn't make requests to the backend (localhost:8000) due to missing CORS headers.

## Solution
Added CORS middleware to the backend to allow cross-origin requests.

## Changes Made

1. **Added `actix-cors` dependency** to `Cargo.toml`
2. **Added CORS middleware** to `src/main.rs`:
   - Allows requests from any origin (for development)
   - Allows all HTTP methods
   - Allows all headers
   - Supports credentials

## Next Steps

**You need to restart your backend server** for the changes to take effect:

1. Stop the current backend (Ctrl+C in the terminal running `cargo run`)
2. Restart it:
   ```bash
   cd /home/vi/airtribe/drafly
   cargo run
   ```

3. The frontend should now be able to make API requests successfully!

## Testing

After restarting, try clicking "Sign in with Google" again. It should work now!

## Security Note

The current CORS configuration allows requests from any origin. For production, you should restrict this to your frontend domain:

```rust
let cors = Cors::default()
    .allowed_origin("https://yourdomain.com")
    .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
    .allowed_headers(vec![
        actix_web::http::header::AUTHORIZATION,
        actix_web::http::header::ACCEPT,
        actix_web::http::header::CONTENT_TYPE,
    ])
    .supports_credentials();
```

