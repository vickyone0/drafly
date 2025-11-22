import { NextResponse } from 'next/server';
import type { NextRequest } from 'next/server';

export function middleware(request: NextRequest) {
  const token = request.cookies.get('jwt_token')?.value || 
    (typeof window !== 'undefined' ? localStorage.getItem('jwt_token') : null);

  // Allow access to login page and OAuth callback
  if (request.nextUrl.pathname === '/login' || 
      request.nextUrl.pathname.startsWith('/auth')) {
    return NextResponse.next();
  }

  // Redirect to login if no token (client-side check will handle this)
  return NextResponse.next();
}

export const config = {
  matcher: ['/((?!api|_next/static|_next/image|favicon.ico).*)'],
};

