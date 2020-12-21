## dclias


### Examples


Get a list of lifetime weapons kills by weapon. (reference_id is the DestinyInventoryItemDefinition manifest hash for the weapon)

```
select reference_id, sum(unique_weapon_kills) as kills from weapon_result group by reference_id order by kills desc
```