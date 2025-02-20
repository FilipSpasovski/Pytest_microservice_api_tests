# vector8 Software Engineer in Test Challenge

Hi,

Welcome to the vector8's coding challenge for Software Engineering in Test position.

## Why

We want to know more about you and your:

- requirement analysis
- critical thinking
- software skills specifically in the automated testing
- bug documentation

## General guidelines

Imagine you are part of a team and others will be working on this or taking it over and you want to be friends.

Write your project as if it was used by others at the end of this day.

## Documentation

Ideally, the code is self-descriptive. If you feel the need to explain implementation choices, or anything else, please insert this as code comments at the most appropriate location.
Project-wide comments, justification, hypothesis or setup instructions can go in a README.

## Description and goal

You are in charge of testing a micro service that serves a machine learning model.

The purpose of the machine learning model is to predict the severity of a car accident based on some input data.
The severity of accident have 4 potential values:

```txt
1. No injuries
2. Minor
3. Major
4. Fatal
```

The machine learning model needs multiple inputs such as weather condition, luminosity, time of the day etc. You can have the full list of input parameters with description in `data.md`.

Given the above requirements, the backend software engineer has delivered to you a prototype software solution as a micro service and you will find it under `backend/` directory. It contains a `README.md` which contains instructions to build and launch the service.

What we are expecting is a set of tests that can be launched to test this micro service. Also, we expect you to list bugs along with their description if you were able to find some. You will also find some sample data to test in the file `inf_data.csv`.

Feel free to code it in your favorite language (), using the framework you want, but provide instructions on how your tests must be launched. Also, do not hesitate to provide remarks (the more critical the better :) on the software code, API design etc.

The coding challenge solution is a good way to showcase your skills so we encourage you to implement some extra components like CI workflow, reporting, etc. depending on your personal preferences (but, of course, it's not obligatory).

The solution should be done in a private GitHub repository with access given to `andrei.letenkov@vector8.com`

## Last words

This coding challenge we have provided to you is considered to be confidential and as such, you may not use or disclose this data and information other than as part of this Coding Challenge.

Thank you for your cooperation and compliance with the above conditions and good luck with the Coding Challenge!
