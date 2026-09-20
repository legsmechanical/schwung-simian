/*
 * SIMIAN UI for Schwung / Ableton Move
 *
 * Ten-voice drum synthesiser, ported from OneTrick SIMIAN 2 (Punk Labs LLC).
 * The root screen is the shared sound-generator surface (kit browser +
 * octave); the parameter tree itself is declared by the plugin's ui_hierarchy
 * and drawn by the Shadow UI, and the voices are seated on the drum grid from
 * the `child_notes` the plugin publishes.
 *
 * GPL-3.0-or-later (DSP: Punk Labs LLC)
 */

import { createSoundGeneratorUI } from '/data/UserData/schwung/shared/sound_generator_ui.mjs';

const ui = createSoundGeneratorUI({
    moduleName: 'SIMIAN',

    onOctaveChange: () => {
        /* The octave shift moves the whole note map, so a voice choked or
         * ringing under the old one has no note left that addresses it. */
        host_module_set_param('all_notes_off', '1');
    },

    /* The status line reads `polyphony` off the DSP: ten voices, one per
     * drum, and never stolen — each one is its own DSP instance. */
    showPolyphony: true,
    showOctave: true,
});

globalThis.init                  = ui.init;
globalThis.tick                  = ui.tick;
globalThis.onMidiMessageInternal = ui.onMidiMessageInternal;
globalThis.onMidiMessageExternal = ui.onMidiMessageExternal;
