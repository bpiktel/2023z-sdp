import { useState } from "react";

const createSample = async (
  name: string,
  azimuth: number,
  elevation: number,
  audio_file: File, // TODO: work out what to do with it
  setCreated: (auth: boolean) => void
): Promise<void> => {
  const { VITE_BASE_API_URL } = import.meta.env;

  // TODO: Implement correct request, specifically - work out how and where to include audio file
  const response = await fetch(`${VITE_BASE_API_URL}/audio`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json"
    },
    body: JSON.stringify({ name, azimuth, elevation }),
    credentials: "include"
  });

  if (response.ok) {
    setCreated(true)
  } else throw new Error("Failed to create sample");
};

const CreateSamplePage = () => {
  const [name, setName] = useState<string>("");
  const [azimuth, setAzimuth] = useState<number>(0);
  const [elevation, setElevation] = useState<number>(0);

  const [audioFile, setAudioFile] = useState<File>()

  const [created, setCreated] = useState<boolean>(false);

  const handleCreate = async () => {
    try {
      if (!audioFile)
        return

      await createSample(name, azimuth, elevation, audioFile, setCreated);
    } catch (error) {
      console.error(error);
    }
  };

  return (
    <div className="flex h-full items-center justify-center">
      <div className="text-center min-w-[16rem]">
        {created ? (
          <h1>Sample uploaded successfully</h1>
        ) : (
          <>
            <h1 className="mb-xl">Upload sample</h1>
            <table className="my-md">
              <tbody>
              <tr>
                <td className="pr-md py-xs">
                  <p className="text-right">Name</p>
                </td>
                <td>
                  <input
                    className="flex-1 px-2"
                    type="string"
                    placeholder="name..."
                    onChange={(e) => setName(e.target.value)}
                  />
                </td>
              </tr>
              <tr>
                <td className="pr-md py-xs">
                  <p className="text-right">Azimuth</p>
                </td>
                <td>
                  <input
                    className="flex-1 px-2"
                    type="number"
                    placeholder="azimuth..."
                    onChange={(e) => setAzimuth(+e.target.value)}
                  />
                </td>
              </tr>
              <tr>
                <td className="pr-md py-xs">
                  <p className="text-right">Elevation</p>
                </td>
                <td>
                  <input
                    className="flex-1 px-2"
                    type="number"
                    placeholder="elevation..."
                    onChange={(e) => setElevation(+e.target.value)}
                  />
                </td>
              </tr>
              </tbody>
            </table>

            <input
              type="file"
              name="file"
              onChange={(e) => {
                setAudioFile(e.target.files?.[0])
              }}
            />

            <div className="flex flex-col mt-lg">
              <button onClick={handleCreate} className="border">
                Create <p className="italic">not working yet!</p>
              </button>
            </div>
          </>
        )}
      </div>
    </div>
  );
};

export default CreateSamplePage;
