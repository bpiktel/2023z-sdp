import { useNavigate, Link } from "@tanstack/react-router";
import { fireAlert } from "components/AlertDialogs";
import { useState } from "react";
import { FaArrowLeft } from "react-icons/fa";
import { ButtonSecondary } from "components/Buttons";
import { FrostedGlass } from "../../components/FrostedGlass.tsx";
import { onEnterDown } from "utils/formUtils.ts";

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
    credentials: "include",
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
    } else fireAlert({ title: "Sample name taken" });
  };

  const handleDegrees = (degrees: number, min: number, max: number, step: number, set?: (n: number) => void) => {
    if (min > max || step < 1) {
      throw new Error(`Invalid arguments: min: ${min}, max: ${max}, step: ${step}`);
    }
    if (degrees < min) {
      degrees = min;
    }
    if (degrees > max) {
      degrees = max;
    }
    const rest = (degrees - min) % step;
    degrees = degrees - rest;
    if (rest / step >= 0.5) {
      degrees += step;
    }
    if (set) {
      set(degrees);
    }
    return degrees;
  };

  const handleCreate = async () => {
    const validatedAzimuth = handleDegrees(azimuth, 0, 345, 15);
    const validatedElevation = handleDegrees(elevation, -90, 90, 15);
    try {
      if (!audioFile) return;

      await createSample(name, validatedAzimuth, validatedElevation, audioFile, onCreated);
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
                  className="w-full flex-1 px-2 py-1"
                  type="text"
                  placeholder="name..."
                  onChange={(e) => setName(e.target.value)}
                  onKeyDown={onEnterDown(handleCreate)}
                />
              </td>
            </tr>
            <tr>
              <td className="pr-md py-xs">
                <p className="text-right">Azimuth</p>
              </td>
              <td className="text-left">
                <input
                  className="flex-1 pl-2 p-1 min-w-24"
                  type="number"
                  defaultValue={0}
                  min="0"
                  max="345"
                  step="15"
                  placeholder="azimuth"
                  onChange={e => handleDegrees(+e.target.value, 0, 345, 15, setAzimuth)}
                  onBlur={e => handleDegrees(+e.target.value, 0, 345, 15, n => { setAzimuth(n); e.target.value = `${n}`; })}
                  onKeyDown={onEnterDown(handleCreate)}
                />
              </td>
            </tr>
            <tr>
              <td className="pr-md py-xs">
                <p className="text-right">Elevation</p>
              </td>
              <td className="text-left">
                <input
                  className="flex-1 pl-2 p-1 min-w-24"
                  type="number"
                  defaultValue={0}
                  min="-90"
                  max="90"
                  step="15"
                  placeholder="elevation"
                  onChange={e => handleDegrees(+e.target.value, -90, 90, 15, setElevation)}
                  onBlur={e => handleDegrees(+e.target.value, -90, 90, 15, n => { setElevation(n); e.target.value = `${n}`; })}
                  onKeyDown={onEnterDown(handleCreate)}
                />
              </td>
            </tr>
          </tbody>
        </table>

        <input
          type="file"
          accept="audio/*"
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
