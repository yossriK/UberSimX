we have a simulation controller, not a real app calling the driver service, this will affect our design a little bit. 
For example, there will be no need for a /accept endpoint where driver chooses to accept or reject a ride after
a mathc been made, we will just assume that all rides are accepted after matcher service finds a match 
between closest driver and rider. 
We also are not too strict on driver filters in terms of car type, luxury, pets allowed, rating etc... 
those details can come in later.

also flags like is driver banned, blocked, cancelled too many rides and now on cooldown status, were not considered initially. 