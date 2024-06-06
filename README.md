# Github Actions Playground

Toy project for exploring the capabilities of Github Actions

## Notes

 - Respository Secrets (not environment secrets) are available like so: `${{ secrets.MY_SECRET }}`
 - Only env vars you explicitly set in the `env` section will be accessible like `echo $MY_ENV_VAR`
 - Repository Variables (not env vars) are available like so: `${{ vars.MY_VAR }}`
