# LaLiga backend monolith (league-svc)
A player system -> linked to users in usersvc
Leagues on any sports
Have all places and what sports they offer mainly
Ranking system for players to find other players to play with
Positive, negative reviews, no-shows, yes-shows

FOCUS ON: Building a user base, the business will come once you are established as a used website, then you can sell place recommendations, ads, group interests, and sell payment service + schedules for the places.


### Todo's for pre-release 0.9
- [x] Change all page parameters to unsigend integers
- [x] Check all page parameters to not be 0
- [x] Change all id paths to unsigned integers
- [x] Fix multiplication overflow panic on page sizes
- [ ] Create deploy script
- [x] Check trust count from SELECT COUNT(*) script
- [ ] Check for already existing player trust each time you create a new one
- [ ] Add trusted player league join request functionality
- [ ] Phone number validation (user-svc)
- [ ] Player fields validation 
- [x] User service error message conversion (Right now we're returning double nested MessageResources. Attempt to parse error message into a single one in this svc (or in communicators))
- [ ] Place submission by users (non approved)
- [ ] Forgor password? (in user-svc)

### Features not for pre-release 0.9
- [ ] User blocking / blacklisting
- [ ] Commending (Basically leaving reviews for players once you played with them)
- [ ] Reputation system (no-shows/shows/no-pays)

### Planned but far off features
- [ ] Teams (you can join league as a team, display teams in your profile page, be in multiple teams)
- [ ] Tournaments (made up of teams)
- [ ] Paying places through you (Payment gateway?)
- [ ] Advertisements for places (Have them pay for ranking)
- [ ] Chats (Group & DM) KEY FEATURE
- [ ] SerruchoTM functionality (League owners can charge from the app with credit cards) ($$$)