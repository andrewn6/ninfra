{ pkgs, ... }:

let
  # PostgreSQL
  postgresql = pkgs.postgresql;
  postgresqlConfig = {
    enable = true;
    package = postgresql;
    dataDir = ./postgresql-data;
    enableTCPIP = true;
    port = 5432;
    user = "postgres";
    password = "admin"; # Change this!
    extraConfig = ''
      # Additional configuration options for PostgreSQL
    '';
  };

  # Redis
  redis = pkgs.redis;
  redisConfig = {
    enable = true;
    package = redis;
    dataDir = ./redis-data;
    port = 6379;
    config = ''
      bind 127.0.0.1
      # Additional configuration options for Redis
    '';
  };
in
{
  services = {
    postgresql = postgresqlConfig;
    redis = redisConfig;
  };
}
