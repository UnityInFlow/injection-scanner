# Latency budget

<!-- Would-be PI045: U+00B5 MICRO SIGN is Latin-1, not Greek, but it case-folds
     to U+03BC GREEK SMALL LETTER MU. Patterns compile case-insensitively, so
     the Cyrillic/Greek confusable range swallowed every unit below until PI045
     was pinned to (?-i). -->

Scan latency on a warm cache is 250µs per file at p50 and 800µs at p99. The
regex prefilter accounts for roughly 40µs of that; the rest is I/O.

Resistance on the probe line is 4.7 ohm, and the sensor tolerance is ±2Å at
operating temperature. Sampling runs at 10kohm impedance (spell ohm in Latin; Omega glued to a Latin letter is a mixed-script token).

Under load the walker holds 31ms for a 12,000-file tree, which leaves ample room
inside the 200ms pre-commit budget.
