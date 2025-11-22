'use client';

import { EmailDetail } from '@/lib/api';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Sparkles } from 'lucide-react';
import { format } from 'date-fns';

interface EmailPreviewProps {
  email: EmailDetail | null;
  onGenerateDraft: (emailId: number) => void;
  isLoading?: boolean;
}

export function EmailPreview({ email, onGenerateDraft, isLoading }: EmailPreviewProps) {
  if (!email) {
    return (
      <div className="flex h-full items-center justify-center">
        <div className="text-center text-muted-foreground">
          <p>Select an email to view</p>
        </div>
      </div>
    );
  }

  return (
    <div className="flex h-full flex-col">
      <Card className="flex-1">
        <CardHeader>
          <div className="flex items-start justify-between">
            <div className="flex-1">
              <CardTitle className="mb-2">{email.subject || 'No subject'}</CardTitle>
              <div className="space-y-1 text-sm text-muted-foreground">
                <p>
                  <span className="font-medium">From:</span> {email.sender || 'Unknown'}
                </p>
                <p>
                  <span className="font-medium">To:</span> {email.to_recipients || 'Unknown'}
                </p>
                <p>
                  <span className="font-medium">Date:</span>{' '}
                  {email.fetched_at
                    ? format(new Date(email.fetched_at), 'PPpp')
                    : 'Unknown'}
                </p>
              </div>
            </div>
            <Button
              onClick={() => onGenerateDraft(email.id)}
              disabled={isLoading}
              className="ml-4"
            >
              <Sparkles className="mr-2 h-4 w-4" />
              Generate Draft
            </Button>
          </div>
        </CardHeader>
        <CardContent>
          <div className="prose max-w-none">
            {email.body_html ? (
              <div
                dangerouslySetInnerHTML={{ __html: email.body_html }}
                className="email-body"
              />
            ) : email.body_text ? (
              <div className="whitespace-pre-wrap text-sm">{email.body_text}</div>
            ) : (
              <p className="text-muted-foreground">No content available</p>
            )}
          </div>
        </CardContent>
      </Card>
    </div>
  );
}

