import { useState } from "react";

const createExperiment = async (
  name: string,
  sample_ids: string[],
  setCreated: (auth: boolean) => void
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
    setCreated(true)
  } else throw new Error("Failed to create experiment");
};

const CreateExperimentPage = () => {
  const [name, setName] = useState<string>("");
  const [sampleIds, setSampleIds] = useState<Array<string>>([]);

  const [created, setCreated] = useState<boolean>(false);

  function addSampleId() {
    setSampleIds((prevState) => [...prevState, ""])
  }

  const handleCreate = async () => {
    try {
      await createExperiment(name, sampleIds, setCreated);
    } catch (error) {
      console.error(error);
    }
  };

  return (
    <div className="flex h-full items-center justify-center">
      <div className="text-center min-w-[16rem]">
        {created ? (
          <h1>Experiment created successfully</h1>
        ) : (
          <>
            <h1>Create experiment</h1>
            <div className="flex flex-col mt-md">
              <div className="flex flex-row">
                <p className="pr-md">Experiment name</p>
                <input
                  className="flex-1 px-2"
                  type="text"
                  placeholder="experiment name..."
                  onChange={(e) => setName(e.target.value)}
                />
              </div>
              <div className="flex flex-col mt-md">
                <h2>Audio samples</h2>
                {sampleIds.length === 0
                  ?
                <p className="italic mt-sm">No samples added yet</p>
                  :
                sampleIds.map((sample, index) => {
                  return <div key={index} className="flex flex-row">
                    <input
                      className="flex-1 mt-sm"
                      placeholder="file name..."
                      value={sample}
                      onChange={(e) => setSampleIds((prevState) => {
                        prevState[index] = e.target.value
                        return [...prevState]
                      })}
                    />
                    <button
                      className="border w-6 h-6 bg-red-600 self-end ml-3"
                      onClick={() => setSampleIds((prevState) => prevState.filter((_, idx) => idx !== index))}
                    >
                      X
                    </button>
                  </div>
                })}
              </div>
              <button onClick={addSampleId} className="border self-center px-sm my-sm">
                Add sample
              </button>
            </div>
            <div className="flex flex-col mt-md">
              <button onClick={handleCreate} className="border">
                Create
              </button>
            </div>
          </>
        )}
      </div>
    </div>
  );
};

export default CreateExperimentPage;
