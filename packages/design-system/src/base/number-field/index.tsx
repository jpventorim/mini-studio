import { Minus, Plus } from "lucide-react";
import { useId, useState } from "react";
import { Button } from "../button";

const displayNumber = new Intl.NumberFormat("en", {
  maximumFractionDigits: 3,
  useGrouping: false,
});

export function NumberField({
  label,
  value,
  min,
  max,
  onChange,
  unit,
  step,
  keyboardStep = step,
  integer = false,
  labelLayout = "above",
}: {
  label: string;
  value: number;
  min: number;
  max: number;
  onChange: (value: number) => void;
  unit?: string;
  step?: number;
  /** Arrow-key increment; defaults to the button step when provided. */
  keyboardStep?: number;
  integer?: boolean;
  labelLayout?: "above" | "inline" | "hidden";
}) {
  const id = useId();
  const [draft, setDraft] = useState<{
    text: string;
    value: number;
    valid: boolean;
  }>();
  const activeDraft = draft?.value === value ? draft : undefined;
  const invalid = activeDraft?.valid === false;
  const hasStepper = step !== undefined && step > 0;
  const describedBy =
    [unit && `${id}-unit`, invalid && `${id}-range`]
      .filter(Boolean)
      .join(" ") || undefined;

  function increment(delta: number) {
    setDraft(undefined);
    onChange(
      Math.max(min, Math.min(max, Number((value + delta).toPrecision(12)))),
    );
  }

  return (
    <div
      className="rf-number-field"
      data-label-layout={labelLayout}
      data-stepper={hasStepper || undefined}
    >
      <label
        className={labelLayout === "hidden" ? "rf-visually-hidden" : "rf-label"}
        htmlFor={id}
      >
        {label}
      </label>
      <div className="rf-number-controls">
        {hasStepper && (
          <Button
            type="button"
            iconOnly
            variant="ghost"
            aria-label={`Decrease ${label.toLowerCase()}`}
            disabled={value <= min}
            onClick={() => increment(-step)}
          >
            <Minus size={16} aria-hidden="true" />
          </Button>
        )}
        <div className="rf-number-box">
          <input
            id={id}
            type="text"
            inputMode={integer ? "numeric" : "decimal"}
            autoComplete="off"
            spellCheck={false}
            maxLength={18}
            value={activeDraft?.text ?? displayNumber.format(value)}
            aria-invalid={invalid || undefined}
            aria-describedby={describedBy}
            onChange={(event) => {
              const text = event.currentTarget.value;
              const number = Number(text.replace(",", "."));
              const valid =
                /^-?\d*(?:[.,]\d*)?$/.test(text) &&
                text !== "" &&
                number >= min &&
                number <= max &&
                (!integer || Number.isInteger(number));
              setDraft({ text, value: valid ? number : value, valid });
              if (valid && number !== value) onChange(number);
            }}
            onBlur={() => setDraft(undefined)}
            onKeyDown={(event) => {
              if (event.key === "Enter") event.currentTarget.blur();
              if (
                keyboardStep !== undefined &&
                keyboardStep > 0 &&
                (event.key === "ArrowUp" || event.key === "ArrowDown")
              ) {
                event.preventDefault();
                increment(
                  event.key === "ArrowUp" ? keyboardStep : -keyboardStep,
                );
              }
            }}
          />
          {unit && <span id={`${id}-unit`}>{unit}</span>}
        </div>
        {hasStepper && (
          <Button
            type="button"
            iconOnly
            variant="ghost"
            aria-label={`Increase ${label.toLowerCase()}`}
            disabled={value >= max}
            onClick={() => increment(step)}
          >
            <Plus size={16} aria-hidden="true" />
          </Button>
        )}
      </div>
      {invalid && (
        <span className="rf-visually-hidden" id={`${id}-range`}>
          Enter {integer ? "a whole number" : "a number"} from {min} to {max}.
        </span>
      )}
    </div>
  );
}
