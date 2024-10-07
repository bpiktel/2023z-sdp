import { useQuery } from "@tanstack/react-query";
import { Link, useParams } from "@tanstack/react-router";
import { FaArrowLeft } from "react-icons/fa";
import { experimentResultListSchema } from "schemas/experimentSchemas";
import { FrostedGlass } from "../../components/FrostedGlass.tsx";
import {
  Sample,
  SampleResult,
  sampleListSchema,
} from "schemas/sampleSchemas.ts";
import { defaultRequestInit } from "utils/fetchUtils.ts";
import { ButtonSecondary } from "components/Buttons.tsx";

const combineResultsWithSample = (
  results: SampleResult[],
  samples: Sample[]
) => {
  return results.map((result) => {
    const matchingSample = samples.find(
      (sample) => sample.id === result.sample_id
    );
    return {
      ...result,
      name: matchingSample?.name,
      sAzimuth: matchingSample?.azimuth,
      sElevation: matchingSample?.elevation,
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

  const { data, isLoading, error } = useQuery({
    queryKey: ["results", id],
    queryFn: getResults,
  });

  const getSamples = () =>
    fetch(`${VITE_BASE_API_URL}/audio/all`, defaultRequestInit)
      .then((res) => res.json())
      .then((data) => sampleListSchema.parse(data));

  const {
    data: samplesData,
    isLoading: samplesLoading,
    error: samplesError
  } = useQuery({
    queryKey: ["samples"],
    queryFn: getSamples,
  });

  const downloadResults = () => {
    const combinedData = data?.map((result) => ({
      username: result.user,
      mode: result.training ? "training" : "test",
      results: combineResultsWithSample(result.sample_results, samplesData!),
    }));

    if (!combinedData) return;

    const headers = [
      "index",
      "username",
      "mode",
      "sample_id",
      "sample_name",
      "sample_azimuth",
      "sample_elevation",
      "answer_azimuth",
      "answer_elevation",
    ];

    let resultString = "";

    resultString += headers.join(";") + "\n";

    combinedData.forEach((testDetails, idx) => {
      testDetails.results.forEach((sampleResult) => {
        resultString += `${idx};${testDetails.username};${testDetails.mode};${sampleResult.sample_id};${sampleResult.name};${sampleResult.sAzimuth};${sampleResult.sElevation};${sampleResult.azimuth};${sampleResult.elevation}\n`;
      });
    });

    const blob = new Blob([resultString], { type: "text/csv" });
    const url = URL.createObjectURL(blob);

    const link = document.createElement("a");
    link.download = `experiment_results.txt`;
    link.href = url;

    link.click();
  };

  if (isLoading || samplesLoading) {
    return <p>Data is loading...</p>;
  }

  if (error || samplesError) {
    return <p>There was an error when fetching your data.</p>;
  }

  return (
    <div className="flex flex-col items-center p-xl max-h-screen">
      <div className="w-full flex justify-between mb-md">
        <Link to="../../" className="flex gap-xs items-center">
          <FaArrowLeft /> Return to experiments page
        </Link>
      </div>
      <FrostedGlass className="flex flex-col items-center min-w-[36rem] h-full">
        <h1>Results</h1>
        <div className="mt-md grid grid-cols-2 gap-sm items-center w-full overflow-y-auto">
          {data?.map((result, idx) => (
            <div
              key={`result_${idx}`}
              className="flex gap-sm items-center justify-center w-full border-b border-white/40 pb-sm"
            >
              <div className="flex flex-col">
                <div className="flex flex-col items-center pb-sm">
                  <div>username: {result.user}</div>
                  <div>mode: {result.training ? "training" : "test"}</div>
                </div>
                {combineResultsWithSample(
                  result.sample_results,
                  samplesData!
                ).map((sampleResult, sIdx) => (
                  <div
                    key={`sample_result_${sIdx}`}
                    className="flex gap-md items-center justify-between text-center w-full"
                  >
                    <div>{sampleResult.name}</div>
                    <div>
                      A: {sampleResult.azimuth}
                      <br />
                      E: {sampleResult.elevation}
                    </div>
                  </div>
                ))}
              </div>
            </div>
          ))}
        </div>
        <div className="mt-lg">
          <ButtonSecondary onClick={downloadResults}>
            Download results
          </ButtonSecondary>
        </div>
      </FrostedGlass>
    </div>
  );
};

export default ExperimentResultsPage;
