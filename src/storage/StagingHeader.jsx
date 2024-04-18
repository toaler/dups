import React, {useState} from 'react'
import styled, { css } from "styled-components";
import CommitIcon from '@mui/icons-material/Commit';

function StagingHeader({ totalBytes }) {
    const bytesReclaimed = 0.0;
    const filesDeleted = 0;
    const filesCompressed = 0;
    const [isPressed, setIsPressed] = useState(false);

    const handleCommitClick = () => {
        setIsPressed(!isPressed);  // Toggle the pressed state
        console.log('Commit button clicked!');
    };

    return <StagingHeaderContainer>
            <div className="flex-container">
                <div className="flex-row">
                    <div className="flex-item">Bytes in scope </div>
                    <div className="flex-item">{totalBytes.toLocaleString("en-US")}</div>
                </div>
                <div className="flex-row">
                    <div className="flex-item">Bytes reclaimed</div>
                    <div className="flex-item">{bytesReclaimed.toLocaleString("en-US")}</div>
                </div>
                <div className="flex-row">
                    <div className="flex-item">Files deleted</div>
                    <div className="flex-item">{filesDeleted.toLocaleString("en-US")}</div>
                </div>
                <div className="flex-row">
                    <div className="flex-item">Bytes compressed</div>
                    <div className="flex-item">{filesCompressed.toLocaleString("en-US")}</div>
                </div>
            </div>


        <StagingCommit
            as="button"  // Treat the StagingCommit as a button
            onClick={handleCommitClick}
            pressed={isPressed}
        >
            <CommitIcon fontSize="large" style={{ fontSize: '60px' }} /> {/* Method 1: Direct fontSize setting */}

        </StagingCommit>


    </StagingHeaderContainer>
}

export default StagingHeader

const StagingHeaderContainer = styled.div`
    display: flex;
    line-height: 24px;
    font-weight: 400;
    color: #FFFFFF;
    overflow-y: hidden;
`;

const StagingCommit = styled.button`
    display: flex;
    justify-content: center;  // Center the icon horizontally inside the button
    align-items: center;      // Center the icon vertically inside the button
    font-size: 4rem;          // Adjust the icon size as needed, consider increasing if necessary
    margin-left: auto;
    margin-right: 30px;
    color: inherit;           // Use the inherited color
    background: none;         // No background
    border: none;             // No border
    ${({ pressed }) => pressed && css`
        color: #BBBBBB; // Change color when pressed
        transform: translateY(2px); // Move down to simulate button press
    `}
`;

// Icon component remains unchanged, assuming the sizing of the icon should also increase if needed
const Icon = ({ type, pressed }) => {
    const IconComponent = type === "commit" ? CommitIcon : null;
    // Setting fontSize directly as a prop
    return <IconComponent style={{ color: pressed ? '#BBBBBB' : undefined }} />;
};