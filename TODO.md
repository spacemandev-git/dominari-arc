# TODO

1. Utility function to get game ids where the authority is a given pubkey
2. Utilty function get all entities owned by a pubkey
3. Way to close a game by destroying the entities in the game
    -> Will require #2
    -> Give authority to Instance Index. 
    -> Cleaning up means calling the delete entity instruction, which wipes from instance index & deletes the entity
4. Give players Score and Kills on Troop Deaths

