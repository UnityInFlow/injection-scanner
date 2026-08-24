# Token and checksum reference

<!-- Would-be PI048, same root cause as deep-package-paths.md: long
     high-entropy strings are ordinary in engineering documentation. -->

A decoded JWT header and payload look like this:

eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkFsZXggUm9lIn0

The release artifact checksum is
e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 and the
previous one was
9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08.

Reverted commits: 4dcc6e79a1b2c3d4e5f60718293a4b5c6d7e8f9012ab34cd and
71cbe8412ab34cd56ef78901a2b3c4d5e6f708190a1b2c3d.

The upload id returned by the API is
AgentTracerUploadSessionIdentifier00000000000000000000000000001.
