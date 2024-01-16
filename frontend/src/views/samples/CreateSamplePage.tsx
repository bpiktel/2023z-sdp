import { useNavigate, Link } from "@tanstack/react-router";
import { fireAlert } from "components/AlertDialogs";
import { useState } from "react";
import { FaArrowLeft } from "react-icons/fa";
import { ButtonSecondary } from "components/Buttons";
import {FrostedGlass} from "../../components/FrostedGlass.tsx";

const createSample = async (
  name: string,
  azimuth: number,
  elevation: number,
  audio_file: File,
  callback: (success: boolean) => void
): Promise<void> => {
  const { VITE_BASE_API_URL } = import.meta.env;

  const formData = new FormData();
  formData.append("", JSON.stringify({ name, azimuth, elevation }));
  formData.append("", audio_file);

  const response = await fetch(`${VITE_BASE_API_URL}/audio`, {
    method: "POST",
    // headers: {"Content-Type": "multipart/form-data"}, #Fun fact. By setting "Content-Type" to "multipart/form-data"
    // you also have to define "boundary" (which postman does on it own), BUT IF YOU SIMPLY DO NOT DEFINE CONTENT-TYPE THE WEB BROWSER WILL DO IT ALL FOR YOU.
    body: formData,
    credentials: "include"
  });

  if (response.ok) {
    callback(true);
    return;
  }
  callback(false);
};

const CreateSamplePage = () => {
  const navigate = useNavigate({ from: "/samples/create" });

  const [name, setName] = useState<string>("");
  const [azimuth, setAzimuth] = useState<number>(0);
  const [elevation, setElevation] = useState<number>(0);

  const [audioFile, setAudioFile] = useState<File>();

  const onCreated = (success: boolean) => {
    if (success) {
      fireAlert({ title: "Sample added" });
      navigate({ to: "/samples" });
    } else fireAlert({ title: "Failed to create experiment" });
  };

  const handleCreate = async () => {
    try {
      if (!audioFile) return;

      await createSample(name, azimuth, elevation, audioFile, onCreated);
    } catch (error) {
      console.error(error);
    }
  };

  return (
    <div className="flex h-full items-center justify-center">
      <div className="absolute left-0 top-0 m-xl">
        <Link to="../" className="flex gap-xs items-center">
          <FaArrowLeft /> Go Back
        </Link>
      </div>
      <FrostedGlass className="text-center min-w-[16rem]">
        <h1 className="mb-xl">Upload sample</h1>
        <table className="my-md">
          <tbody>
            <tr>
              <td className="pr-md py-xs">
                <p className="text-right">Name</p>
              </td>
              <td>
                <input
                  className="flex-1 px-2 py-1"
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
                  className="flex-1 px-2 py-1"
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
                  className="flex-1 px-2 py-1"
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
            setAudioFile(e.target.files?.[0]);
          }}
        />
        <div className="flex flex-col mt-lg">
          <ButtonSecondary onClick={handleCreate} className="border">
            Create
          </ButtonSecondary>
        </div>
      </FrostedGlass>
    </div>
  );
};

export default CreateSamplePage;
