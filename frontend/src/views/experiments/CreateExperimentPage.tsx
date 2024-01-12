import { useQuery } from "@tanstack/react-query";
import { Link, useNavigate } from "@tanstack/react-router";
import { fireAlert } from "components/AlertDialogs";
import { ButtonSecondary } from "components/Buttons";
import { useState } from "react";
import { FaArrowLeft, FaMinus, FaPlus } from "react-icons/fa";
import { Sample, sampleListSchema } from "schemas/sampleSchemas";
import SamplePreviewWidget from "views/samples/SamplePreviewWidget";

const createExperiment = async (
  name: string,
  sample_ids: string[],
  callback: (success: boolean) => void
): Promise<void> => {
  const { VITE_BASE_API_URL } = import.meta.env;

  const response = await fetch(`${VITE_BASE_API_URL}/experiments`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json"
    },
    body: JSON.stringify({ name, sample_ids }),
    credentials: "include"
  });

  if (response.ok) {
    callback(true);
    return;
  }

  callback(false);
};

const CreateExperimentPage = () => {
  const navigate = useNavigate({ from: "/experiments/create" });

  const [name, setName] = useState<string>("");
  const [sampleIds, setSampleIds] = useState<Array<string>>([]);

  const addSample = (id: string) => {
    setSampleIds((prevState) => [...prevState, id]);
  };

  const removeSample = (id: string) => {
    setSampleIds((prevState) => prevState.filter((sId) => sId !== id));
  };

  const onCreated = (success: boolean) => {
    if (success) {
      fireAlert({ title: "Experiment created" });
      navigate({ to: "/experiments" });
    } else fireAlert({ title: "Failed to create experiment" });
  };

  const handleCreate = async () => {
    try {
      await createExperiment(name, sampleIds, onCreated);
    } catch (error) {
      console.error(error);
    }
  };

  return (
    <div className="flex flex-col items-center p-xl gap-xl h-screen overflow-x-hidden max-h-screen">
      <div className="absolute left-0 top-0 m-xl">
        <Link to="../" className="flex gap-xs items-center">
          <FaArrowLeft /> Go back
        </Link>
      </div>
      <h1>Create experiment</h1>
      <div className="flex flex-row">
        <p className="pr-md">Experiment name</p>
        <input
          className="flex-1 px-2"
          type="text"
          placeholder="experiment name..."
          onChange={(e) => setName(e.target.value)}
        />
      </div>
      <div className="">
        <ButtonSecondary
          onClick={handleCreate}
          disabled={sampleIds.length === 0 || name.length === 0}
        >
          Create
        </ButtonSecondary>
      </div>
      <AudioSelector
        selectedSampleIds={sampleIds}
        addSample={addSample}
        removeSample={removeSample}
      />
    </div>
  );
};

const AudioSelector = ({
  selectedSampleIds,
  addSample,
  removeSample
}: {
  selectedSampleIds: string[];
  addSample: (id: string) => void;
  removeSample: (id: string) => void;
}) => {
  const { VITE_BASE_API_URL } = import.meta.env;

  const getSamples = () =>
    fetch(`${VITE_BASE_API_URL}/audio/all`)
      .then((res) => res.json())
      .then((data) => sampleListSchema.parse(data));

  const { data, isLoading, isFetching, error } = useQuery({
    queryKey: ["samples"],
    queryFn: getSamples
  });

  if (isLoading || data == null) {
    return <p>Data is loading...</p>;
  }

  if (isFetching) {
    return <p>Data is fetching...</p>;
  }

  if (error) {
    return <p>There was an error when fetching your data.</p>;
  }

  const selectedSamples: Sample[] = selectedSampleIds
    .map((id) => data.find((sample) => sample.id.id.String === id))
    .filter((o) => o !== undefined) as Sample[];

  const unselectedSamples: Sample[] = data.filter(
    (sample) => !selectedSampleIds.includes(sample.id.id.String)
  );

  return (
    <div className="flex flex-row gap-xl">
      <div className="">
        <h3 className="mb-sm">Selected samples</h3>
        {selectedSamples.map((sample) => (
          <div
            key={sample.id.id.String}
            className="flex items-center justify-between gap-sm py-sm border-b last:border-0 border-white/60"
          >
            <SamplePreviewWidget sample={sample} />
            <div>
              <FaMinus
                className="cursor-pointer"
                onClick={() => removeSample(sample.id.id.String)}
              />
            </div>
          </div>
        ))}
      </div>
      <div className="">
        <h3 className="mb-sm">Available samples</h3>
        {unselectedSamples.map((sample) => (
          <div
            key={sample.id.id.String}
            className="flex items-center justify-between gap-sm py-sm border-b last:border-0 border-white/60"
          >
            <SamplePreviewWidget sample={sample} />
            <div>
              <FaPlus
                className="cursor-pointer"
                onClick={() => addSample(sample.id.id.String)}
              />
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};

export default CreateExperimentPage;
