'use client';

import { useEffect } from 'react';
import { useRouter, usePathname } from 'next/navigation';

export function AuthGuard({ children }: { children: React.ReactNode }) {
  const router = useRouter();
  const pathname = usePathname();

  useEffect(() => {
    // Skip auth check for login page
    if (pathname === '/login') {
      return;
    }

    // Check for JWT token
    const token = localStorage.getItem('jwt_token');
    if (!token) {
      router.push('/login');
    }
  }, [router, pathname]);

  // Don't render children if on login page and have token (redirect to home)
  useEffect(() => {
    if (pathname === '/login') {
      const token = localStorage.getItem('jwt_token');
      if (token) {
        router.push('/');
      }
    }
  }, [pathname, router]);

  return <>{children}</>;
}

