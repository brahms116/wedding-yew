# Wedding website frontend

This is a website built with the [Yew framework](https://yew.rs/), it is pretty much a rust clone of react,
 well at least the functional component interface. 

### Features

- Rsvp for invitees. Allows invitees to rsvp for the wedding & reception.
- Personalisation. Each invitee will recieve a unique link corresponding with their invitee id, a UUID v4 ID. 
Upon visiting the website with the correct id in the url query parameter, the website is populated with the invitees names and information
and an rsvp page is available to them.
For general visitors without an ID, a general public view of the website is presented
- Time limited live-stream link. On the day of the wedding day, a cta-button and navigation link will appear to allow users
to navigate to the live-stream

### Backend infrastructure

The website is connected to an AWS lambda function via an AWS Api Gateway.
The function code and IaC can be found [here](https://github.com/brahms116/wedding_funcs).

### Styling

This site uses [Tailwindcss](https://tailwindcss.com) for styling, hence some npm and node dependencies.

### Deployment and environments

There are 2 deployment environments configured, DEV and PROD, these represents the respective branches in git. 
The site is deployed via github actions to Netlify.
These workflow files can be found in the `.github/workflows` directory.

Here are the respective links to the sites using the Netlify App domain.

- [DEV](https://wedding-development.netlify.app/)
- [PROD](https://wedding-production.netlify.app/)

### TODOS

- General clean up of handling errors, some errors are not used, decide how these errors are displayed
Currently there is only a "blue-screen" error message which crashes the app
- Fix all cargo linting errors
- Enforce consistent logic handling. Some pages and components have controllers whereas some pages do not
