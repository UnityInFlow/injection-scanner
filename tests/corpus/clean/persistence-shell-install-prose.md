# Installing a command-line tool

<!-- CAT-03 false-positive specimen (phase 05, plan 05-02). Decision it
     defends: D-02 / PI070 / PI071. Editing a shell profile, a crontab or an
     authorized_keys file is the most common step in any install guide, and
     it is addressed to the person at the keyboard in the second person. The
     shipped PI070 excludes `you` as an addressee for exactly this reason;
     a later persistence-object pattern in this range must not undo that.
     Sources, per sentence:
       - the two "shell config" sentences are SOURCED, copied as quoted in
         05-RESEARCH.md section Q1 (fetched there from
         docs.brew.sh/Installation and from the nvm-sh/nvm README on
         raw.githubusercontent.com); they were not re-fetched for this plan.
       - the SSH and cron sentences are SYNTHESIZED in the register of any
         SSH or cron tutorial. No third-party page is quoted for them. -->

## Shell setup

You need to update your shell's config file (which file exactly depends on your shell, for example ~/.bashrc or ~/.zshrc) to include this:

Now add these lines to your ~/.bashrc, ~/.profile, or ~/.zshrc file to have it automatically sourced upon login:

## SSH access

To log in without a password you should add your public key to ~/.ssh/authorized_keys on the server, then check the file is only writable by you.

## Scheduled runs

Run `crontab -e` and add the following line to your crontab so the cleanup job runs every night at two.
