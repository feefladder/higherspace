2. Given this environment, which libraries can be created or improved upon?
   - Tiff2 reader
     - get geotiff to work with it ASAP
     - put in bevy_terrain
     - [rgis](https://github.com/frewsxcv/rgis/issues/124) is very interested
     - show off example of loading a tiff directly from Zenodo
   - Bevy erosion model
     - base off of richdem
     - automated data aquisition:
       - IsdaSoil for getting soil data
       - this weather service for historical weather data
       - rosetta3 (api) for pedotransfer functions on aggregated data (so as not to overload their servers)
         - compare results of aggregate vs non-aggregate soil data?
         - there was also this much bigger R model for pedotransfer functions
      - optional: FAO56

Additional: make self-explanatory
![](../img/gate.mp4)
