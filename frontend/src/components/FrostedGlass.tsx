import { ReactNode } from "react";

type frostedWithTheme = {
  children: ReactNode;
  className?: string;
  theme?: "light" | "dark" | "overlay";
};

export const FrostedGlass = ({
  children,
  className,
  theme,
}: frostedWithTheme) => {
  switch (theme) {
    case "dark":
    default:
      return (
        <div
          className={`rounded-3xl p-xl backdrop-blur-md bg-gradient-to-br from-black/[.7] to-black/[0.4] ${className}`}
        >
          {children}
        </div>
      );
    case "light":
      return (
        <div
          className={`rounded-3xl p-xl backdrop-blur-md bg-gradient-to-br from-white/[.7] to-white/[0.4] ${className}`}
        >
          {children}
        </div>
      );
    case "overlay":
      return (
        <div className={`rounded-3xl p-lg bg-gray-500/[.2] ${className}`}>
          {children}
        </div>
      );
  }
};
