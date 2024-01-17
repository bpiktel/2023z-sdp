import { useQuery } from "@tanstack/react-query";
import { Link, useParams } from "@tanstack/react-router";
import { FaArrowLeft } from "react-icons/fa";
import { experimentResultListSchema } from "schemas/experimentSchemas";
import { FrostedGlass } from "../../components/FrostedGlass.tsx";
import {
  Sample,
  SampleResult,
  sampleListSchema
} from "schemas/sampleSchemas.ts";
import { defaultRequestInit } from "utils/fetchUtils.ts";

const combineResultsWithSample = (
  results: SampleResult[],
  samples: Sample[]
) => {
  return results.map((result) => {
    return {
      ...result,
      name: samples.find((sample) => sample.id.id.String === result.sample_id)
        ?.name
    };
  });
};

const ExperimentResultsPage = () => {
  const { VITE_BASE_API_URL } = import.meta.env;
  const { id } = useParams({ strict: false });

  const getResults = () =>
    fetch(`${VITE_BASE_API_URL}/experiments/results/${id}`, defaultRequestInit)
      .then((res) => res.json())
      .then((data) => experimentResultListSchema.parse(data));

  const { data, isLoading, isFetching, error } = useQuery({
    queryKey: ["results", id],
    queryFn: getResults
  });

  const getSamples = () =>
    fetch(`${VITE_BASE_API_URL}/audio/all`, defaultRequestInit)
      .then((res) => res.json())
      .then((data) => sampleListSchema.parse(data));

  const {
    data: samplesData,
    isLoading: samplesLoading,
    isFetching: samplesFetching,
    error: samplesError
  } = useQuery({
    queryKey: ["samples"],
    queryFn: getSamples
  });

  console.log(data);

  if (isLoading || isFetching || samplesLoading || samplesFetching) {
    return <p>Data is loading...</p>;
  }

  if (error || samplesError) {
    return <p>There was an error when fetching your data.</p>;
  }

  console.log("data");
  console.log(data);
  console.log(samplesData);
  console.log("===========");

  console.log(samplesData);
  console.log(combineResultsWithSample(data![0].sample_results, samplesData!));

  return (
    <div className="flex flex-col items-center p-xl">
      <div className="w-full flex justify-between mb-md">
        <Link to="../../" className="flex gap-xs items-center">
          <FaArrowLeft /> Return to experiments page
        </Link>
      </div>
      <FrostedGlass className="flex flex-col items-center">
        <h1>Results</h1>
        <div className="mt-md flex flex-col gap-sm items-center w-full">
          {data?.map((result, idx) => (
            <div
              key={`result_${idx}`}
              className="flex gap-sm items-center w-full border-b border-white/40 pb-sm"
            >
              <div>{idx + 1}</div>
              <div className="flex flex-col">
                {combineResultsWithSample(
                  result.sample_results,
                  samplesData!
                ).map((sampleResult, sIdx) => (
                  <div
                    key={`sample_result_${sIdx}`}
                    className="flex gap-sm items-center w-full"
                  >
                    <div>{sampleResult.name}</div>
                    <div>
                      Azimuth: {sampleResult.azimuth}
                      <br />
                      Elevation: {sampleResult.elevation}
                    </div>
                  </div>
                ))}
              </div>
            </div>
          ))}
        </div>
      </FrostedGlass>
    </div>
  );
};

export default ExperimentResultsPage;
