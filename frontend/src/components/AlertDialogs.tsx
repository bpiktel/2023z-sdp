import Swal, { SweetAlertResult } from "sweetalert2";

const swalCustomStyle = Swal.mixin({
  customClass: {
    popup: "rounded-none",
    title: "h2 text-center text-black",
    htmlContainer: "h4 text-center text-black mt-sm mb-md",
    confirmButton:
      "flex items-center justify-center rounded-sm px-md py-4 text-primary hover:text-white hover:bg-primary outline outline-2 outline-primary -outline-offset-1 mb-md mt-md",
    cancelButton:
      "flex items-center justify-center rounded-sm px-md py-4 bg-primary hover:bg-primary-600 text-white outline outline-2 outline-primary mr-md -outline-offset-1 mb-md mt-md"
  },
  buttonsStyling: false,
  allowOutsideClick: false,
  allowEscapeKey: true,
  allowEnterKey: true
});

const Toast = Swal.mixin({
  customClass: {
    title: "h2 text-center text-black"
  },
  toast: true,
  position: "top-end",
  showConfirmButton: false,
  timer: 2000,
  timerProgressBar: true,
  didOpen: (toast) => {
    toast.addEventListener("mouseenter", Swal.stopTimer);
    toast.addEventListener("mouseleave", Swal.resumeTimer);
  }
});

export const fireConfirmationModal = async ({
  title,
  body,
  confirmText,
  cancelText
}: {
  title: string;
  body: string;
  confirmText?: string;
  cancelText?: string;
}): Promise<SweetAlertResult> => {
  const onClickEnterConfirm = (event: KeyboardEvent): void => {
    if (event.key === "Enter") {
      swalCustomStyle.clickConfirm();
    }
  };

  return await swalCustomStyle.fire({
    title,
    text: body,
    showCancelButton: true,
    confirmButtonText: confirmText ?? "Confirm",
    cancelButtonText: cancelText ?? "Cancel",
    reverseButtons: true,
    keydownListenerCapture: true,
    stopKeydownPropagation: true,
    allowEscapeKey: true,
    allowEnterKey: true,
    preConfirm: () => {
      window.removeEventListener("keyup", onClickEnterConfirm);

      return true;
    },
    didOpen: () => {
      setTimeout(
        () => window.addEventListener("keyup", onClickEnterConfirm),
        100
      );
    },
    didClose: () => {
      window.removeEventListener("keyup", onClickEnterConfirm);
    }
  });
};

export async function fireAlert({
  title
}: {
  title: string;
}): Promise<SweetAlertResult> {
  return await Toast.fire({
    title
  });
}
