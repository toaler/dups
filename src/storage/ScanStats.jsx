import React, {useState} from 'react';
import styled from "styled-components"
import DirectionsRunIcon from '@mui/icons-material/DirectionsRun';

const formatElapsedTime = (elapsedTime) => {
    // Convert elapsed time to hours, minutes, and milliseconds
    const hours = Math.floor(elapsedTime / 3600000); // Total hours
    const minutes = Math.floor((elapsedTime % 3600000) / 60000); // Remaining minutes
    const seconds = Math.floor((elapsedTime % 60000) / 1000); // Convert remainder to seconds
    const milliseconds = elapsedTime % 1000; // Milliseconds are the remainder of elapsed time divided by 1000

    // Format milliseconds to ensure it's always displayed as a three-digit number
    return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}.${milliseconds.toString().padStart(3, '0')}`;
};

function ScanStats({status, elapsedTime, resources, directories, files, size}) {

    return <ScanStatsContainer>
            <div className="flex-container">
                <div className="flex-row">
                    <div className="flex-item">Status</div>
                    <div className="flex-item">{status}</div>
                </div>
                <div className="flex-row">
                    <div className="flex-item">Elapsed</div>
                    <div className="flex-item">{formatElapsedTime(elapsedTime)}</div>
                </div>
                <div className="flex-row">
                    <div className="flex-item">Resources</div>
                    <div className="flex-item">{Number(resources).toLocaleString()}</div>
                </div>
                <div className="flex-row">
                    <div className="flex-item">Directories</div>
                    <div className="flex-item">{Number(directories).toLocaleString()}</div>
                </div>
                <div className="flex-row">
                    <div className="flex-item">Files</div>
                    <div className="flex-item">{Number(files).toLocaleString()}</div>
                </div>
                <div className="flex-row">
                    <div className="flex-item">Size</div>
                    <div className="flex-item">{Number(size).toLocaleString()}</div>
                </div>
            </div>
    </ScanStatsContainer>
}

export default ScanStats

const ScanStatsContainer = styled.div`
    display: flex;
    line-height: 24px;
    font-weight: 400;
    color: #FFFFFF;
    overflow-y: hidden; 
`;
