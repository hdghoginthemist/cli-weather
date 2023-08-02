
# Weather Forecast

Weather Forecast app in your CLI.

## Prerequisites

To run it you'll need an API key from https://www.weatherapi.com

Free account allows you to get up to 3 days worth of weaher forecast.



## Environment Variables

To run this project, you will need to export the `API_KEY` environment variable in your CLI session:

Generate the key from www.weatherapi.com and export it in your CLI by running ```export API_KEY=XXXXXXX```

## Arguments

There are two arguments you can specify 
- number of days for the forecast `-d`
- location `-l` 

If you choose not to specify the argumnets, the app will default to 1 days forecast in the city of Amsterdam, The Netherlands.


## Run Locally

Clone the project

```bash
  git clone git@github.com:hdghoginthemist/cli-weather.git
```

Go to the project directory

```bash
  cd cli-weather
```

You can run this project using `cargo run`

```bash
  cargo run -q -- -d 3 -l Chicago
```

or build the binary using `cargo build --release` and run it

```bash
  ./target/release/forecast -d 3 -l Chicago
```


## Lessons Learned

Build this app while following Tim McNamara's course "How to learn rust". 
