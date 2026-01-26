type CircularProgressConfig = {
  container: HTMLElement;
  sizeInPx?: number;
  strokeWidthInPx?: number;
  trackColor?: string;
  progressColor?: string;
};

const DEFAULT_SIZE_IN_PX = 200;
const DEFAULT_STROKE_WIDTH_IN_PX = 8;
const DEFAULT_TRACK_COLOR = "rgba(0, 0, 0, 0.1)";
const DEFAULT_PROGRESS_COLOR = "#e74c3c";
const SVG_NAMESPACE = "http://www.w3.org/2000/svg";

const createSvgElement = <K extends keyof SVGElementTagNameMap>(
  tagName: K
): SVGElementTagNameMap[K] => document.createElementNS(SVG_NAMESPACE, tagName);

const setAttributes = (
  element: SVGElement,
  attributes: Record<string, string | number>
) => {
  for (const [key, value] of Object.entries(attributes)) {
    element.setAttribute(key, String(value));
  }
};

export const createCircularProgress = (config: CircularProgressConfig) => {
  const {
    container,
    sizeInPx = DEFAULT_SIZE_IN_PX,
    strokeWidthInPx = DEFAULT_STROKE_WIDTH_IN_PX,
    trackColor = DEFAULT_TRACK_COLOR,
    progressColor = DEFAULT_PROGRESS_COLOR,
  } = config;

  const radius = (sizeInPx - strokeWidthInPx) / 2;
  const circumference = 2 * Math.PI * radius;
  const center = sizeInPx / 2;

  const svg = createSvgElement("svg");
  setAttributes(svg, {
    width: sizeInPx,
    height: sizeInPx,
    viewBox: `0 0 ${sizeInPx} ${sizeInPx}`,
    class: "circular-progress",
  });

  const trackCircle = createSvgElement("circle");
  setAttributes(trackCircle, {
    cx: center,
    cy: center,
    r: radius,
    fill: "none",
    stroke: trackColor,
    "stroke-width": strokeWidthInPx,
  });

  const progressCircle = createSvgElement("circle");
  setAttributes(progressCircle, {
    cx: center,
    cy: center,
    r: radius,
    fill: "none",
    stroke: progressColor,
    "stroke-width": strokeWidthInPx,
    "stroke-linecap": "round",
    "stroke-dasharray": circumference,
    "stroke-dashoffset": circumference,
    transform: `rotate(-90 ${center} ${center})`,
  });
  progressCircle.classList.add("circular-progress__bar");

  svg.appendChild(trackCircle);
  svg.appendChild(progressCircle);
  container.appendChild(svg);

  const setProgress = (progress: number) => {
    const clampedProgress = Math.max(0, Math.min(1, progress));
    const offset = circumference * (1 - clampedProgress);
    progressCircle.setAttribute("stroke-dashoffset", String(offset));
  };

  const setProgressColor = (color: string) => {
    progressCircle.setAttribute("stroke", color);
  };

  const destroy = () => {
    svg.remove();
  };

  return {
    element: svg,
    setProgress,
    setProgressColor,
    destroy,
  };
};

export type CircularProgress = ReturnType<typeof createCircularProgress>;
