# frozen_string_literal: true

def num_unique_emails(emails)
  emails.map { |email| email[0, email.index(/[+@]/)].gsub('.', '') + email[email.index('@')..] }.uniq.size
end
