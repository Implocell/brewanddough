import {createSignal, createEffect, Show} from 'solid-js'
import {createStore} from 'solid-js/store'
import { SolidApexCharts } from 'solid-apexcharts';
import {ApexOptions} from 'apexcharts'

interface BarChartOptions {
    chart: {
        id: string
    };
    xaxis: {
        categories: number[]
    }
}

interface BarChartSeries {
    list: {
        name:string;
        data: number[];
    }[]
}

export const BarChart = () => {
    const [series, setSeries] =createSignal<BarChartSeries>()
    const [options, setOptions] = createSignal<BarChartOptions>()

    createEffect(() => {
        const fetchData = async () => {
          const res = await fetch("http://api.openweathermap.org/geo/1.0/direct?q=London&limit=5&appid=2c36818bb5ca9e829313dd736fd15859");
          const data = await res.json();
        

          setOptions({
            chart: {
              id: 'solidchart-example',
            },
            xaxis: {
                categories: data.map((d:{name:string}) => (d.name))
            },
          });
    
    
          setSeries({
            list: [
              {
                name: 'series-1',
                data: data.map((d: {lat:number}) => (d.lat))
              },
            ]
          });
        }


        setTimeout(() => fetchData(), 3000)
        
    });



      const hasData = () => {
        if(series() !== undefined && options() !== undefined){
            return true
        }

        return false
      }
        return <Show when={hasData()} fallback={<div>Loading...</div>}><SolidApexCharts width="1200" type="bar" options={options()!} series={series()!.list}  /></Show>

}

