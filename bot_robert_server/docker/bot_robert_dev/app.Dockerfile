FROM rust:1.60

# Rebuild code on each start to apply code changes.
CMD cargo install --offline --path /var/app/bot_robert_server/ ; bot_robert_server
