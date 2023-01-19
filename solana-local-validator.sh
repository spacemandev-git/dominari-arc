#!/bin/bash

solana-test-validator --bind-address 0.0.0.0 \
--mint 83vu98TvDWstyexssmc3FmyN2KCjbSQf6sVYcger6Rxg \
--url https://api.devnet.solana.com \
--clone 2TfB33aLaneQb5TNVwyDz3jSZXS6jdW2ARw1Dgf84XCG `# switchboardProgramId` \
--clone J4CArpsbrZqu1axqQ4AnrqREs3jwoyA1M5LMiQQmAzB9 `# switchboardProgramDataAddress` \
--clone CKwZcshn4XDvhaWVH9EXnk3iu19t6t5xP2Sy2pD6TRDp `# switchboardIdlAddress` \
--clone BYM81n8HvTJuqZU1PmTVcwZ9G8uoji7FKM6EaPkwphPt `# switchboardProgramState` \
--clone FVLfR6C2ckZhbSwBzZY4CX7YBcddUSge5BNeGQv5eKhy `# switchboardVault` \
--clone So11111111111111111111111111111111111111112 `# switchboardMint` \
--clone DLLuutmefBgsuPob8M6JTMxyMR7b8Q2VmCDEjkKTTcP5 `# tokenWallet` \
--clone 4tDrH3XfhkUfkkEiAoRTNub8T7fasbQpd7oRMoqU8iyT `# oracleQueue` \
--clone 83vu98TvDWstyexssmc3FmyN2KCjbSQf6sVYcger6Rxg `# oracleQueueAuthority` \
--clone 3WPc1CX2fMbP6g6HhUxJMk74sLDaiDR9VXZhU9RGDYmR `# oracleQueueBuffer` \
--clone 267xeWfEobpP1UfezjtmXF4nGzsRFLXakgaiH3WAAA2c `# crank` \
--clone Bp9CNwMYHAGDc1Amf5Q7q2ARiAoybWjAhEPxC2JsGMJs `# crankBuffer` \
--clone FR1kjNVxGzB9pzicJm2efwjjwFjuY6q6KddoHsFoQMWH `# oracle` \
--clone 83vu98TvDWstyexssmc3FmyN2KCjbSQf6sVYcger6Rxg `# oracleAuthority` \
--clone FsZayJCEV8bLVtRjEEX7FcKdMDDby9Q4FB6j53bZhAaK `# oracleEscrow` \
--clone ATGqnWbJvUbgUNSTdH3cz4etMekcuLtnGanHL6CyRyPY `# oraclePermissions` 