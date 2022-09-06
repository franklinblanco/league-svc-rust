This project should have multiple things
A player system -> linked to users in usersvc
Leagues on any sports
Have all places and what sports they offer mainly
Ranking system for players to find other players to play with
Positive, negative reviews, no-shows, yes-shows

FUTURE: Payment system

FOCUS ON: Building a user base, the business will come once you are established as a used website, then you can sell place recommendations, ads, user data, and sell payment service + schedules for the places, and digital payment for these places.

### Todo's for pre-release 0.9
- [x] Change all page parameters to unsigend integers
- [x] Check all page parameters to not be 0
- [x] Change all id paths to unsigned integers
- [x] Fix multiplication overflow panic on page sizes
- [ ] Create deploy script
- [x] Check trust count from SELECT COUNT(*) script
- [ ] Check for already existing player trust each time you create a new one
- [ ] Add trusted player league join request functionality
- [ ] Phone number validation
- [ ] Player fields validation 
- [ ] User service error message conversion (Right now we're returning double nested MessageResources. Attempt to parse error message into a single one in this svc (or in communicators))

### Features not for pre-release 0.9
