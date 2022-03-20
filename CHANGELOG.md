v0.0.5
--------------
- Add Asynchronous request.
- Added module. `nonblocking`
- Added struct. `nonblocking::Client` `nonblocking::OAuth`

v0.0.5
--------------
- Changed the return value at Client.call() Result<Value, String> to Result<Response, Error>.
- Added functions. `people` `organizations` `series` `characters` `casts` `staffs`

v0.0.4
---------------
- Fix accesstoken include query.
- Added functions. `me_following_activities` `me_reviews` `me` `following` `followers` `activities` `reviews` `users`

v0.0.3
---------------
- Fix docs.rs/annis build.

v0.0.2
---------------
- Added enums to types accept by argument of Service.params(). `Works` `Episodes` `Records` `MeStatuses` `MeRecords` `MeWorks` `MePrograms`   

v0.0.1
===============
Initial release