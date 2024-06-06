# pregate

This service is designed to receive (language) API requests (or any HTTP POST requests) via `/api/v1` and check the patterns in the `pregate-rules.toml` for keywords to trigger a fixed response instead of a (generative) response. Set the pregate-rules.toml, see the pregate-rules.toml_EXAMPLE, populating it with the URL, auth information (API token), as well as the rules and hard-coded response.

