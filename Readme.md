# LaLiga backend monolith (league-svc)
A player system -> linked to users in usersvc
Leagues on any sports
Have all places and what sports they offer mainly
Ranking system for players to find other players to play with
Positive, negative reviews, no-shows, yes-shows

**FOCUS ON**: Building a user base, the business will come once you are established as a used website, then you can sell place recommendations, ads, group interests, and sell payment service + schedules for the places.


### Todo's for pre-release 0.9
- [x] Change all page parameters to unsigend integers
- [x] Check all page parameters to not be 0
- [x] Change all id paths to unsigned integers
- [x] Fix multiplication overflow panic on page sizes
- [x] Create deploy script
- [x] Check trust count from SELECT COUNT(*) script
- [x] Check for already existing player trust each time you create a new one
- [x] Add trusted player league join request functionality
- [x] Plan leaving leagues
- [ ] Phone number validation (user-svc)
- [ ] Player fields validation 
- [x] User service error message conversion (Right now we're returning double nested MessageResources. Attempt to parse error message into a single one in this svc (or in communicators))
- [ ] Place submission by users (non approved)
- [ ] Forgor password? (in user-svc)
- [ ] Create big dto containing League, place, and all players in league (plus queue (number of Unaccepted join requests)) and make an endpoint to retrieve this from the frontend
- [ ] GetPlayer endpoint should return complete user profile (or at least login + create user)
- [x] Age groups (Kind of already done)
- [x] UTC timezone instead of naivedatetime
- [ ] Interests on signup and changeable (Users have to tell us what their 1-3 favorite sports are)
- [x] Chats (Group & DM) KEY FEATURE
- [ ] Player inviting players to league
- [ ] Sport selection & Location Selection Methods

#### Bugs left to fix
- [ ] Make sure one player can't make 2 leagues at the same time. (maybe by replacing the previous one or deleting)
- [x] Joining a league should be possible as the owner of the league
- [x] If a player is kicked out of a league he shouldn't be able to join it again unless invited 
- [ ] Return what state the league player is in on the get_all_leagues player has applied to
- [ ] Get all players in league endpoint returns players even if they're kicked
- [ ] Require a token for all endpoints (As this is now a mobile app not a webpage)


### Features not for pre-release 0.9
- [ ] User blocking / blacklisting
- [ ] Commending (Basically leaving reviews for players once you played with them)
- [ ] Reputation system (no-shows/shows/no-pays)

### Planned but far off features
- [ ] Teams (you can join league as a team, display teams in your profile page, be in multiple teams)
- [ ] Tournaments (made up of teams)
- [ ] Paying places through you (Payment gateway?)
- [ ] Advertisements for places (Have them pay for ranking)
- [ ] SerruchoTM functionality (League owners can charge from the app with credit cards) ($$$)
- [ ] ELO point system like chess