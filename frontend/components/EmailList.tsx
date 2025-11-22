'use client';

import { Email } from '@/lib/api';
import { format } from 'date-fns';
import { cn } from '@/lib/utils';

interface EmailListProps {
  emails: Email[];
  selectedEmailId: number | null;
  onSelectEmail: (email: Email) => void;
  isLoading?: boolean;
}

export function EmailList({ emails, selectedEmailId, onSelectEmail, isLoading }: EmailListProps) {
  if (isLoading) {
    return (
      <div className="flex h-full items-center justify-center">
        <div className="text-muted-foreground">Loading emails...</div>
      </div>
    );
  }

  if (emails.length === 0) {
    return (
      <div className="flex h-full items-center justify-center">
        <div className="text-center">
          <p className="text-muted-foreground">No emails found</p>
          <p className="mt-2 text-sm text-muted-foreground">
            Click "Fetch Unread" to load emails from Gmail
          </p>
        </div>
      </div>
    );
  }

  return (
    <div className="flex h-full flex-col overflow-y-auto">
      {emails.map((email) => (
        <button
          key={email.id}
          onClick={() => onSelectEmail(email)}
          className={cn(
            'flex flex-col gap-2 border-b p-4 text-left transition-colors hover:bg-accent',
            selectedEmailId === email.id && 'bg-accent'
          )}
        >
          <div className="flex items-start justify-between">
            <div className="flex-1 truncate">
              <p className="truncate font-semibold">{email.sender || 'Unknown'}</p>
              <p className="truncate text-sm text-muted-foreground">
                {email.subject || 'No subject'}
              </p>
            </div>
            <span className="ml-2 text-xs text-muted-foreground">
              {email.fetched_at
                ? format(new Date(email.fetched_at), 'MMM d')
                : ''}
            </span>
          </div>
          {email.snippet && (
            <p className="line-clamp-2 text-sm text-muted-foreground">
              {email.snippet}
            </p>
          )}
        </button>
      ))}
    </div>
  );
}

