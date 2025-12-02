'use client';

import { Suspense } from 'react';
import { useEffect, useState } from 'react';
import { useRouter, useSearchParams } from 'next/navigation';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { api } from '@/lib/api';
import { Mail } from 'lucide-react';

// Create a separate component that uses useSearchParams
function LoginContent() {
  const router = useRouter();
  const searchParams = useSearchParams();
  const [isLoading, setIsLoading] = useState(false);

  useEffect(() => {
    // Check if we have a token from backend redirect (after OAuth callback)
    const token = searchParams.get('token');
    const email = searchParams.get('email');

    if (token && email) {
      // Token received from backend redirect
      setIsLoading(true);
      localStorage.setItem('jwt_token', token);
      localStorage.setItem('user_email', decodeURIComponent(email));
      // Clear URL params and redirect
      router.replace('/');
      return;
    }

    // Check if we have a code from OAuth callback (direct callback - shouldn't happen but handle it)
    const code = searchParams.get('code');
    const state = searchParams.get('state');

    if (code && state) {
      handleCallback(code, state);
    }
  }, [searchParams, router]);

  const handleLogin = async () => {
    try {
      setIsLoading(true);
      console.log('Starting Google authentication...');
      const { auth_url } = await api.startGoogleAuth();
      console.log('Got auth URL, redirecting...', auth_url);
      // Store state for verification
      window.location.href = auth_url;
    } catch (error) {
      console.error('Failed to start auth:', error);
      const errorMessage = error instanceof Error ? error.message : 'Failed to start authentication';
      alert(`Authentication Error: ${errorMessage}`);
      setIsLoading(false);
    }
  };

  const handleCallback = async (code: string, state: string) => {
    try {
      setIsLoading(true);
      const { jwt, email } = await api.googleCallback(code, state);
      localStorage.setItem('jwt_token', jwt);
      localStorage.setItem('user_email', email);
      router.push('/');
    } catch (error) {
      console.error('Failed to complete auth:', error);
      alert('Failed to complete authentication');
      setIsLoading(false);
    }
  };

  return (
    <div className="flex min-h-screen items-center justify-center bg-background">
      <Card className="w-full max-w-md">
        <CardHeader className="text-center">
          <div className="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-full bg-primary">
            <Mail className="h-6 w-6 text-primary-foreground" />
          </div>
          <CardTitle className="text-2xl">Welcome to Drafly</CardTitle>
          <CardDescription>
            AI-powered email assistant for professional communication
          </CardDescription>
        </CardHeader>
        <CardContent>
          {isLoading ? (
            <div className="text-center">
              <div className="mb-4 text-sm text-muted-foreground">
                {searchParams.get('token') ? 'Completing login...' : 'Connecting to Google...'}
              </div>
            </div>
          ) : (
            <Button
              onClick={handleLogin}
              disabled={isLoading}
              className="w-full"
              size="lg"
            >
              Sign in with Google
            </Button>
          )}
        </CardContent>
      </Card>
    </div>
  );
}

// Main page component with Suspense boundary
export default function LoginPage() {
  return (
    <Suspense fallback={
      <div className="flex min-h-screen items-center justify-center bg-background">
        <Card className="w-full max-w-md">
          <CardHeader className="text-center">
            <div className="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-full bg-primary">
              <Mail className="h-6 w-6 text-primary-foreground" />
            </div>
            <CardTitle className="text-2xl">Welcome to Drafly</CardTitle>
            <CardDescription>
              AI-powered email assistant for professional communication
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div className="text-center">
              <div className="mb-4 text-sm text-muted-foreground">
                Loading...
              </div>
            </div>
          </CardContent>
        </Card>
      </div>
    }>
      <LoginContent />
    </Suspense>
  );
}