# Source file conventions

<!-- CAT-03 false-positive specimen (phase 05, plan 05-02). Decision it
     defends: D-02. A self-propagation pattern is grammatically identical to
     a licence-header or file-template rule -- both tell a reader, in the
     second person, to put something in every file it creates -- and the two
     differ only in the object. These are the rules scaffolding documents
     really contain: a licence header, a copyright notice, a file template, a
     docstring requirement. Each names a non-instruction object, so a
     self-propagation pattern must discriminate on the propagated object and
     not on the addressee.
     Sources: ALL SYNTHESIZED. 05-RESEARCH.md section Q1 located real
     AGENTS.md copyright-header rules only through search summaries, which it
     marks MEDIUM and says must be fetched before committing a specimen; they
     were not fetched, so no line here is attributed to a third party. The
     first two sentences are the legitimate analogues 05-CONTEXT.md D-02 names
     verbatim. -->

## Headers

Include the license header in every file you create.

Add the standard copyright notice to any new file you create.

## Templates

New command modules start from `templates/command.rs`, which already carries the module docstring, the error type and the test stub.

Copy the file template into each new package you scaffold, then rename the placeholder identifiers.

## Documentation

Every public function you add must have a docstring that states its arguments, its return value and the errors it can produce.

Write a short changelog entry for each user-visible change and put it under the Unreleased heading.
