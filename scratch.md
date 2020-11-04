https://www.bungie.net/Platform/Destiny2/1/Profile/4611686018429783292/?components=100%2C200
///Destiny2/SearchDestinyPlayer/{membershipType}/{displayName}/ 

curl -i -H "Accept: application/json" -H "Content-Type: application/json" http://hostname/resource


2305843009264966986
 String path =
        "https://www.bungie.net/Platform/Destiny2/1/Account/4611686018429783292/Character/-1/Stats/?modes=5&periodType=1&groups=1,2,3";

        final String _apiBase = "https://www.bungie.net";


        https://www.bungie.net/Platform/Destiny2/1/Account/4611686018429783292/Character/0/Stats/?modes=5&groups=1,2,3

header
        Client-ID

          DAY          WEEK        MONTH      ALL TIME
     [ W  L  K/D ][ W  L  K/D ][ W  L  K/D ][ W  L  K/D ]
ALL
COMP
TRIALS
IB

(have each row a color, with a gradient going down)

#!/bin/bash

DATA=$(curl -H "X-API-Key: $API_KEY" "https://www.bungie.net/Platform/Destiny2/1/Account/4611686018429783292/Character/0/Stats/?modes=5&groups=1,2,3")

GAMES_WON=$(echo $DATA | jq -r '.Response.allPvP.allTime.activitiesWon.basic.displayValue')

GAMES_TOTAL=$(echo $DATA | jq -r '.Response.allPvP.allTime.activitiesEntered.basic.displayValue')

GAMES_LOST=$((GAMES_TOTAL - GAMES_WON))

#WIN_PERCENT=$((GAMES_WON / GAMES_TOTAL))

WIN_PERCENT=$(bc <<<"scale=2; $GAMES_WON / $GAMES_TOTAL")
WIN_PERCENT=$(bc <<<"scale=2; $WIN_PERCENT * 100")
WIN_PERCENT=${WIN_PERCENT%.*}

echo W:$GAMES_WON L:$GAMES_LOST $WIN_PERCENT%

