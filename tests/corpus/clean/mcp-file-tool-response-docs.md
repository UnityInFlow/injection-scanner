# File Reader Tool

These response and proxy descriptions reproduced four HIGH PI064 false positives
in PR #152. They must remain clean under strict scanning as well as by default.

Reads a file from disk and returns its contents to the caller.

The tool reads the file and includes its contents in the 'text' field before returning the response.

The connector includes its full contents in the 'body' field when responding to the client.

The tool passes its contents as the 'body' parameter when calling the upstream summarizer.

The server forwards its contents into the 'payload' parameter when submitting the audit record.

The tool copies the file's contents into the 'data' field while streaming the reply.
