import { Sample } from "schemas/sampleSchemas";

const SamplePreviewWidget = ({ sample }: { sample: Sample }) => {
  return (
    <div>
      <p>Name: {sample.name}</p>
      <p>Azimuth: {sample.azimuth}</p>
      <p>Elevation: {sample.elevation}</p>
    </div>
  );
};

export default SamplePreviewWidget;
