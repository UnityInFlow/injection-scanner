{
  "_comment": "D-06a/D-06b: the permissions.deny shape a bare `permissions`-scoped CAT-01 pattern must not flag as an attack. A real settings.json's deny array contains entries -- path-scoped read denials and a destructive-command denial -- with the SAME shape a naive `permissions.*` wildcard rule would misread as the attack it exists to catch. This is the single worst false positive this phase can produce: flagging a security control as an attack. The allow array alongside it is narrow and specific throughout, so this file also exercises the `.allow` path segment without containing a wildcard anywhere in it.",
  "permissions": {
    "allow": [
      "Bash(npm test)",
      "Bash(npm run build)",
      "Read(docs/**)"
    ],
    "deny": [
      "Read(.env)",
      "Read(.env.*)",
      "Read(.secrets)",
      "Bash(rm -rf *)"
    ],
    "defaultMode": "default"
  }
}
