# Check if a custom user has been set, otherwise default to 'postgres'
$dbUser = if ($null -ne $env:DB_USER) { $env:DB_USER } else { "postgres" };
# Check if a custom password has been set, otherwise default to 'password'
$dbPassword = if ($null -ne $env:DB_PASSWORD) { $env:DB_PASSWORD } else { "password" };
# Check if a custom database name has been set, otherwise default to 'newsletter'
$dbName = if ($null -ne $env:DB_NAME) { $env:DB_NAME } else { "newsletter" };
# Check if a custom port has been set, otherwise default to '5432'
$dbPort = if ($null -ne $env:DB_PORT) { $env:DB_PORT } else { "5432" };

# Launch postgres using Docker
if (!$env:SKIP_DOCKER) {
    docker run `
        --name postgres `
        -e POSTGRES_USER=$dbUser `
        -e POSTGRES_PASSWORD=$dbPassword `
        -e POSTGRES_DB=$dbName `
        -p "${dbPort}:5432" `
        -d postgres -N 1000
    # ^ Increased maximum number of connections for testing purposes
}

Start-Sleep 3

$env:DATABASE_URL = "postgres://${dbUser}:${dbPassword}@localhost:${dbPort}/${dbName}";
sqlx database create
sqlx migrate run